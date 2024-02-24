import { cleanskyinitiative_backend } from "../../declarations/cleanskyinitiative_backend";

export const a = 10;
const btn_getCity = document.querySelector(".btn__getCity");
const pollutionInfo = document.querySelector(".pollution__info");

btn_getCity.addEventListener("click", getCity);

async function getCity(e) {
  e.preventDefault();

  btn_getCity.setAttribute("disabled", true); // Butonu disabled yap
  const cityText = document.querySelector(".input__getCity").value.toString();

  await get_events_city_from_api(cityText);

  btn_getCity.removeAttribute("disabled"); // Butonu tekrar aktif yap

  return false;
}

async function get_events_city_from_api(cityText) {
  try {
    const api_data = await cleanskyinitiative_backend.get_events_city_from_api(
      cityText
    );
    console.log(api_data);
    pollutionInfo.textContent = api_data;
  } catch (error) {
    console.error("Error:", error);
    pollutionInfo.textContent = "Error fetching data"; // Hata durumunda kullanıcıya bildir
  }
}

const btn_donate = document.querySelector(".btn_donate");
btn_donate.addEventListener("click", () => {
  alert("Coming soon ...");
});

// document.querySelector("form").addEventListener("submit", async (e) => {
//   e.preventDefault();
//   const button = e.target.querySelector("button");

//   const name = document.getElementById("name").value.toString();

//   button.setAttribute("disabled", true);

//   // Interact with foo actor, calling the greet method
//   const greeting = await cleanskyinitiative_backend.greet(name);

//   button.removeAttribute("disabled");

//   document.getElementById("greeting").innerText = greeting;

//   return false;
// });

// cleanskyinitiative_backend.get_events_city_from_api
