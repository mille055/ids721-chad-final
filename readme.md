# Patient-Friendly Radiology Report Converter
# IDS 721 Spring 2025

Radiology reports can be filled with complex medical terms that are difficult for the average patient to understand. This project provides a simple web application that summarizes radiology report findings into patient-friendly language using a locally hosted large language model (LLM). It also highlights common medical terms in the original text and displays their definitions as hover-over tooltips. The definitions are drawn from a custom-built dictionary tailored for this project (small now, but could grow), offering quick and accessible explanations for patients. 

Built with:
- **Rust** using the Actix-Web framework (backend + minimal frontend)
- **Mozilla's [llamafile]** (https://github.com/Mozilla-Ocho/llamafile) to serve a quantized model (e.g. DeepSeek 8B or Phi-2)
- **Docker & Docker Compose** for local orchestration
- **HTML & JS** frontend with inline display and term highlighting

---

## 🧬 LLM Architecture and EC2 Tradeoffs

### Model Selection
- **DeepSeek 8B** (quantized): Good performance and coherence; requires a larger instance (`g4dn.xlarge` or better).
- **Phi-2** (smaller, more efficient): Works on smaller machines but has lower-quality output in clinical contexts.
- **TinyLLaMA / Other LLMs**: Fast but overly generic summaries. Often too vague for diagnostic nuance.

### EC2 Considerations
| Model        | EC2 Type       | Pros                          | Cons                              |
|--------------|----------------|-------------------------------|-----------------------------------|
| DeepSeek 8B  | g4dn.xlarge+   | Quality summaries, good scale | Cost, startup time, GPU needed    |
| Phi-2        | t2.large+      | Fast setup, cheaper           | Generic output, less informative  |

**Takeaway:** Tradeoff between speed/cost and summary quality. A larger instance may be necessary for reliable clinical-grade rewriting.

---

## ✨ Project Structure
The tree structure is shown [here](tree.txt)

---
## 🌐 App Preview

### 📷 Screenshots
- ![img](static/images/projectf_app1.png)

After the user presses convert, the text appears with defined terms in bold text and blue color; when you hover over the words, you get a small box with the definition, such as this one for ascites (cursor not shown):

- ![img](static/images/projectf_app_hover.png)

The llamafile is used to summarize the original report with patient-friendly text. While the model is thinking, the user sees:

- ![img](static/images/projectf_app_summarywaiting.png)

And eventually the model will populate the patient-friendly summary text:

- ![img](static/images/projectf_app_summary_old.png)

### 🎬 Demo Video
- [Demo video link placeholder](https://your.video.url/here)

---

## 🚀 How to Run Locally

### 1. Clone and prepare the project

Make sure you have:

- Rust installed (`rustup`)
- Docker installed (`docker`, `docker-compose`)

```bash
git clone https://gitlab.com/dukeaiml/ids721-spring2025/cm-final1.git
```

Download and place the following into the `llamafile/` directory:
- The **deepseek-8b.llamafile** model (download from Hugging Face)

---

### 2. Build and Start with Docker Compose

From the project root:

```bash
docker build --platform linux/amd64 -t mille055/projectf:latest .
docker run --platform linux/amd64 -p 8000:8000 mille055/projectf:latest
```

### 3. Access and use the app

Rust Actix app will be available at: http://localhost:8000

Llamafile model server will run internally on port 8080 (not directly exposed)

✅ Paste radiology findings into the form.

✅ Hover over highlighted words in the text to see definitions.

✅ Wait for the model to deliver a summary in patient-friendly terms.


🧠 How It Works

Frontend: Very simple HTML form served by Actix.
Backend: POSTs findings to the /explain endpoint.
Rust App: Formats a prompt, calls the Llamafile model via HTTP (http://llama:8080/completion).
Llamafile Server: Uses a open-source HuggingFace llamafile model (such as quantized DeepSeek 8B model) to generate patient-friendly text.

## 🛠 EC2 Deployment

To deploy:

Provision an EC2 instance (e.g., g4dn.xlarge for DeepSeek, or t2.large for Phi-2).
Download a .llamafile model and set executable permissions:
```bash
wget https://huggingface.co/.../phi-2.llamafile -O phi-2.llamafile
chmod +x phi-2.llamafile
```
Copy the the provided script restart_all.st to the EC2 instance. Then, run it:
```bash
./restart_all.sh
```

This script:

- Stops existing app containers and llamafile processes
- Launches the llamafile server in the background (logs to phi2.log)
- Runs the Docker container using --network host so the app can reach the LLM

See restart_all.sh for details.

## 📦 Deployment Notes

This app has been deployed to:

AWS EC2 (with persistent EBS volumes)

On the EC2 instance, I have installed a .sh file (restart_all.sh), which stops other llamafile model and running docker containers with the app, and restarts the model and the app's docker container with the appropriate settings for the app to be able to access the model. An example is shown in the repository under restart_all.sh. 

## 📃 LLM Use Disclosure

Parts of this code and documentation were developed using GitHub Copilot and ChatGPT to assist with:

- Rust syntax
- Documentation
- CICD 

**All clinical outputs should be reviewed by licensed professionals. This tool is intended for research and educational purposes only.**

## Author

Built by Chad Miller - 2025