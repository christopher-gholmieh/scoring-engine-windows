// Functions:
function generate_generation_date() {
    // Variables (Assignment):
    // Current:
    const current_date = new Date();

    // Year:
    const current_year = current_date.getFullYear();

    // Month:
    const current_month = String(current_date.getMonth() + 1).padStart(2, "0");

    // Day:
    const current_day = String(current_date.getDate()).padStart(2, "0");

    // Hours:
    const hours = String(current_date.getHours()).padStart(2, "0");

    // Minutes:
    const minutes = String(current_date.getMinutes()).padStart(2, "0");

    // Seconds:
    const seconds = String(current_date.getSeconds()).padStart(2, "0");

    // Timezone:
    const timezone = Intl.DateTimeFormat("en-US", {
        timeZoneName: "short"
    }).formatToParts(current_date).find(part => part.type === "timeZoneName").value;

    // Logic:
    document.getElementById("generation").innerText = `Report Generated At ${current_year}/${current_month}/${current_day} ${hours}:${minutes}:${seconds} ${timezone}`;
}

function generate_api_data() {
    fetch("http://localhost:8080/api").then(response => {
        if (response.ok == false) {
            throw new Error(`[!] HTTP Error: ${response.status}`)
        }

        return response.json();
    }).then(data => {
        // Title:
        const title = document.getElementById("image-title");
        title.innerHTML = data.image_title;

        // Connection:
        const status = document.getElementById("status");
        status.innerHTML = "Connection Status: <span class=\"green\">OK"

        // Total:
        document.getElementById("total-points").innerText = `${data.points} out of ${data.total_points} points received`

        // Penalties:
        document.getElementById("penalties-label").innerText = `${data.penalties.length} penalties assessed, for a loss of ${data.penalty_points || 0} points:`

        data.penalties.forEach(penalty => {
            // Variables (Assignment):
            // Text:
            const text = document.createElement("p");

            // HTML:
            text.className = "penalty";

            // Content:
            text.innerText = penalty;

            // Logic:
            document.getElementById("penalties").appendChild(text);
        })

        // Remediations:
        document.getElementById("remediations-label").innerText = `${data.remediations.length} out of ${data.number_vulnerabilities} scored security issues fixed, for a gain of ${data.points} points:`

        data.remediations.forEach(remediation => {
            // Variables (Assignment):
            // Text:
            const text = document.createElement("p");

            // Content:
            text.innerText = remediation;

            // Logic:
            document.getElementById("remediations").appendChild(text);
        })
    }).catch(error => {
        console.log(`[*] API Error: ${error}`)
    })
}

function generate_binary() {
    // Variables (Assignment):
    // Binaries:
    const binaries = document.querySelectorAll('.binary');

    // Logic:
    binaries.forEach(element => {
        // Variables (Assignment):
        // Base:
        const base = element.dataset.binary || '1010 1100';

        // Logic:
        element.dataset.binary = Array(10000).fill(base).join(' ');
    });    
}

// Logic:
document.addEventListener('DOMContentLoaded', () => {
    generate_generation_date();
    generate_api_data();
    generate_binary();
});
