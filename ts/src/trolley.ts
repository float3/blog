const NUM = 63
document.addEventListener("DOMContentLoaded", async () => {
  const trolleyPath = "/misc/trolley/"
  const extensions = ["mp4", "jpg"]
  const randomNumber = Math.floor(Math.random() * NUM) + 1

  const randomNum = String(randomNumber).padStart(2, "0")

  const mp4Src = `${trolleyPath}${randomNum}.${extensions[0]}`
  const jpgSrc = `${trolleyPath}${randomNum}.${extensions[1]}`

  const response = await fetch(jpgSrc)
  const selectedSrc = response.ok ? jpgSrc : mp4Src
  window.location.href = selectedSrc
})
