# Patient-Friendly Radiology Report Converter

This project provides a simple web application that rewrites radiology report findings into patient-friendly language using a local LLM model server.

Built with:
- Rust (Actix-Web framework) for the backend and minimal frontend
- Mozilla's [llamafile](https://github.com/Mozilla-Ocho/llamafile) to serve a quantized DeepSeek 8B model
- Docker and Docker Compose to orchestrate the containers

---

## ✨ Project Structure

---

## 🚀 How to Run Locally

### 1. Clone and prepare the project

Make sure you have:

- Rust installed (`rustup`)
- Docker installed (`docker`, `docker-compose`)

Download and place the following into the `llamafile/` directory:
- The **llamafile** binary (download from GitHub releases)
- The **deepseek-8b.llamafile** model (download from Hugging Face)

---

### 2. Build and Start with Docker Compose

From the project root:

```bash
docker-compose build
docker-compose up
```

Rust Actix app will be available at: http://localhost:8000
Llamafile model server will run internally on port 8080 (not directly exposed)
✅ Paste radiology findings into the form.
✅ Get a patient-friendly explanation instantly.

🧠 How It Works

Frontend: Very simple HTML form served by Actix.
Backend: POSTs findings to the /explain endpoint.
Rust App: Formats a prompt, calls the Llamafile model via HTTP (http://llama:8080/completion).
Llamafile Server: Uses a quantized DeepSeek 8B model to generate patient-friendly text.
Docker Compose ensures both services talk to each other correctly by internal networking (llama hostname).

🛠 Updating the Model or App


Task	Action
Update the Actix app (code change)	Rebuild only the app container
Update the model (new .llamafile)	Rebuild only the llamafile container
They are cleanly separated for fast updates!

⚡ Useful Commands

Build everything:
docker-compose build
Run services:
docker-compose up
Run services in background (detached):
docker-compose up -d
Stop and remove containers:
docker-compose down
📦 Deployment (Coming Soon)

This setup is ready to be deployed on:



✍️ Author

Built by [Chad Miller] - 2025