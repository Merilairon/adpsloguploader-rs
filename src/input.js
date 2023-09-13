const fileInput = document.querySelector("#folder input[type=file]");
fileInput.onchange = () => {
  if (fileInput.files.length > 0) {
    const fileName = document.querySelector("#folder .file-name");
    fileName.textContent = fileInput.value;
  }
};
