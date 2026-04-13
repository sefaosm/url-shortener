const API = "/api/v1";
let currentPage = 1;

// --- DOM References ---
const shortenForm = document.getElementById("shorten-form");
const urlInput = document.getElementById("url-input");
const shortenBtn = document.getElementById("shorten-btn");
const resultDiv = document.getElementById("result");
const shortUrlText = document.getElementById("short-url-text");
const shortUrlLink = document.getElementById("short-url-link");
const errorMsg = document.getElementById("error-msg");
const urlsBody = document.getElementById("urls-body");
const emptyState = document.getElementById("empty-state");
const paginationDiv = document.getElementById("pagination");
const statsModal = document.getElementById("stats-modal");
const statsBody = document.getElementById("stats-body");

// --- Shorten ---
shortenForm.addEventListener("submit", async (e) => {
    e.preventDefault();
    resultDiv.classList.add("hidden");
    errorMsg.classList.add("hidden");
    shortenBtn.disabled = true;
    shortenBtn.textContent = "Shortening...";

    try {
        const res = await fetch(`${API}/shorten`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ url: urlInput.value.trim() }),
        });

        const data = await res.json();

        if (!res.ok) {
            errorMsg.textContent = data.message || data.error || "Something went wrong";
            errorMsg.classList.remove("hidden");
            return;
        }

        shortUrlText.textContent = data.short_url;
        shortUrlLink.href = data.short_url;
        resultDiv.classList.remove("hidden");
        urlInput.value = "";
        loadUrls();
    } catch (err) {
        errorMsg.textContent = "Network error. Is the server running?";
        errorMsg.classList.remove("hidden");
    } finally {
        shortenBtn.disabled = false;
        shortenBtn.textContent = "Shorten";
    }
});

// --- Copy ---
function copyShortUrl() {
    navigator.clipboard.writeText(shortUrlText.textContent).then(() => {
        const btn = document.querySelector(".copy-btn");
        btn.textContent = "Copied!";
        setTimeout(() => { btn.textContent = "Copy"; }, 1500);
    });
}

// --- Load URLs ---
async function loadUrls(page = 1) {
    currentPage = page;

    try {
        const res = await fetch(`${API}/urls?page=${page}&per_page=10`);
        const data = await res.json();

        if (data.urls.length === 0) {
            urlsBody.innerHTML = "";
            emptyState.classList.remove("hidden");
            paginationDiv.innerHTML = "";
            return;
        }

        emptyState.classList.add("hidden");
        urlsBody.innerHTML = data.urls.map((u) => `
            <tr>
                <td class="code-cell">${u.short_code}</td>
                <td class="url-cell" title="${escapeHtml(u.original_url)}">${escapeHtml(u.original_url)}</td>
                <td class="click-count">${u.click_count}</td>
                <td>${formatDate(u.created_at)}</td>
                <td>
                    <button class="action-btn stats-btn" onclick="showStats('${u.short_code}')">Stats</button>
                    <button class="action-btn delete-btn" onclick="deleteUrl('${u.short_code}')">Delete</button>
                </td>
            </tr>
        `).join("");

        renderPagination(data.page, data.total_pages);
    } catch (err) {
        urlsBody.innerHTML = "";
        emptyState.textContent = "Failed to load URLs.";
        emptyState.classList.remove("hidden");
    }
}

// --- Pagination ---
function renderPagination(current, totalPages) {
    if (totalPages <= 1) {
        paginationDiv.innerHTML = "";
        return;
    }

    let html = "";
    for (let i = 1; i <= totalPages; i++) {
        const active = i === current ? "active" : "";
        html += `<button class="page-btn ${active}" onclick="loadUrls(${i})">${i}</button>`;
    }
    paginationDiv.innerHTML = html;
}

// --- Stats Modal ---
async function showStats(code) {
    statsBody.innerHTML = "<p style='color:#94a3b8'>Loading...</p>";
    statsModal.classList.remove("hidden");

    try {
        const res = await fetch(`${API}/stats/${code}`);
        if (!res.ok) {
            statsBody.innerHTML = "<p style='color:#fca5a5'>Failed to load stats.</p>";
            return;
        }

        const s = await res.json();

        let html = `
            <div class="stat-row"><span class="stat-label">Short Code</span><span class="stat-value">${s.short_code}</span></div>
            <div class="stat-row"><span class="stat-label">Original URL</span><span class="stat-value" title="${escapeHtml(s.original_url)}">${escapeHtml(s.original_url)}</span></div>
            <div class="stat-row"><span class="stat-label">Click Count</span><span class="stat-value">${s.click_count}</span></div>
            <div class="stat-row"><span class="stat-label">Created</span><span class="stat-value">${formatDate(s.created_at)}</span></div>
            <div class="stat-row"><span class="stat-label">Expires</span><span class="stat-value">${s.expires_at ? formatDate(s.expires_at) : "Never"}</span></div>
            <div class="stat-row"><span class="stat-label">Last Clicked</span><span class="stat-value">${s.last_clicked_at ? formatDate(s.last_clicked_at) : "Never"}</span></div>
            <div class="stat-row"><span class="stat-label">Active</span><span class="stat-value">${s.is_active ? "✅ Yes" : "❌ No"}</span></div>
        `;

        if (s.recent_clicks && s.recent_clicks.length > 0) {
            html += `<p class="clicks-title">Recent Clicks (${s.recent_clicks.length})</p>`;
            html += s.recent_clicks.map((c) => `
                <div class="click-item">
                    ${formatDate(c.clicked_at)} · ${escapeHtml(c.ip_address || "unknown")} · ${escapeHtml(truncate(c.user_agent || "unknown", 60))}
                </div>
            `).join("");
        } else {
            html += `<p class="clicks-title">No clicks yet</p>`;
        }

        statsBody.innerHTML = html;
    } catch (err) {
        statsBody.innerHTML = "<p style='color:#fca5a5'>Network error.</p>";
    }
}

function closeStats() {
    statsModal.classList.add("hidden");
}

// --- Delete ---
async function deleteUrl(code) {
    if (!confirm(`Delete short URL "${code}"?`)) return;

    try {
        const res = await fetch(`${API}/urls/${code}`, { method: "DELETE" });

        if (res.ok || res.status === 204) {
            loadUrls(currentPage);
        } else {
            alert("Failed to delete URL.");
        }
    } catch (err) {
        alert("Network error.");
    }
}

// --- Helpers ---
function escapeHtml(str) {
    const div = document.createElement("div");
    div.textContent = str;
    return div.innerHTML;
}

function formatDate(iso) {
    const d = new Date(iso);
    return d.toLocaleDateString() + " " + d.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
}

function truncate(str, max) {
    return str.length > max ? str.substring(0, max) + "..." : str;
}

// --- Init ---
loadUrls();