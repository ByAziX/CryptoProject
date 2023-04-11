document.getElementById("generateOTP").addEventListener("click", function() {
  // Appeler l'API pour générer l'OTP
  fetch("/generate_otp")
  .then(response => response.json())
});