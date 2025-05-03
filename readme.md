# Patient-Friendly Radiology Report Converter
# IDS 721 Spring 2025

Radiology reports can be filled with complex medical terms that are difficult for the average patient to understand. This project provides a simple web application that summarizes radiology report findings into patient-friendly language using a locally hosted large language model (LLM). It also highlights common medical terms in the original text and displays their definitions as hover-over tooltips. The definitions are drawn from a custom-built dictionary tailored for this project (small now, but could grow), offering quick and accessible explanations for patients. 

Built with:
- **Rust** using the **Actix-Web** framework — powers both the backend and minimal frontend.
- **Mozilla's llamafile** (https://github.com/Mozilla-Ocho/llamafile) to serve a quantized model (e.g. DeepSeek 8B or Phi-2)
- **Docker** used to containerize and deploy the Rust application.
- **AWS EC2**, used to host both the application and model in a secure, scalable environment.

---

## 🌐 App Preview

### The application is deployed on AWS EC2 and can be found at : [http://54.159.67.65:8000](http://54.159.67.65:8000)

### 📷 Screenshots
- ![img](static/images/new_projectf_app1.png)

After the user presses convert, the text appears with defined terms in bold text and blue color; when you hover over the words, you get a small box with the definition, such as this one for ascites (cursor not shown):

- ![img](static/images/new_projectf_app_hover.png)

The llamafile is used to summarize the original report with patient-friendly text. While the model is thinking, the user sees:

- ![img](static/images/projectf_app_summarywaiting.png)

And eventually the model will populate the patient-friendly summary text:

- ![img](static/images/new_projectf_app_summary1.png)

### 🎬 Demo Video
- [Demo video link placeholder](https://youtu.be/7D-Dc0Q3Ins)

---

## 🚀 How to Run Locally

### 1. Clone and prepare the project

Make sure you have:

- Rust installed (`rustup`)
- Docker installed (`docker`, `docker-compose`)

```bash
git clone https://gitlab.com/dukeaiml/ids721-spring2025/cm-final1.git
```

### 2. Download and Prepare Llamafile (Phi-2)

Download and place the following into the `llamafile/` directory:
- A llamafile model, such as **deepseek-8b.llamafile** or **phi-2**
- For example, go to https://huggingface.co/jartine/phi-2-llamafile
- Click the Download button next to phi-2.llamafile (you must be signed into Hugging Face).
- Place the downloaded file into your EC2 or project directory.
- Rename the binary or update the LLAMAFILE_BINARY path in restart_all.sh accordingly.
- Make it executable:

```bash
chmod +x phi-2.llamafile
```
---

### 3. Build and Start app with Docker Compose

From the project root:

```bash
cargo build --release

docker build --platform linux/amd64 -t mille055/projectf:latest .
docker run --platform linux/amd64 -p 8000:8000 mille055/projectf:latest
```

### 4. Access and use the app

Rust Actix app will be available at: http://localhost:8000

Llamafile model server will run internally on port 8080 (not directly exposed)

✅ Paste radiology findings into the form.

✅ Hover over highlighted words in the text to see definitions.

✅ Wait for the model to deliver a summary in patient-friendly terms.

## 🛠 EC2 Deployment

For deployment:

- Provision an EC2 instance (e.g., g4dn.xlarge for DeepSeek, or t2.large for Phi-2).
- Download a .llamafile model and set executable permissions as described above:
- Copy the the provided script restart_all.st to the EC2 instance. 
- Then, run it:
```bash
chmod +x restart_all.sh
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

## 🧠 How It Works

Frontend: Very simple HTML form served by Actix.
Backend: POSTs findings to the /explain endpoint.
Rust App: Formats a prompt, calls the Llamafile model via HTTP (http://llama:8080/completion).
Llamafile Server: Uses a open-source HuggingFace llamafile model (such as quantized DeepSeek 8B model) to generate patient-friendly text.

## Monitoring and Metrics

There are separate logs for the llamafile model and for the dockerized app which can be accessed in the ec2 instance, and which are also visible in CloudWatch. 

Llama log in Cloudwatch:
![img](static/images/llama_log.png)

Log for the Dockerized app:
![img](static/images/app_log.png)

This provides, among other things, an elapsed time for the request to return from the model after pressing the 'convert' button. 

Additionally, there is a /metrics route in the app, which provides a simple counter for the number of requests processed. 

## 📃 LLM Use Disclosure

Parts of this code and documentation were developed using GitHub Copilot and ChatGPT to assist with:

- Rust syntax
- Documentation
- CICD 

**All clinical outputs should be reviewed by licensed professionals. This tool is intended for research and educational purposes only.**

## Author

Built by Chad Miller - 2025