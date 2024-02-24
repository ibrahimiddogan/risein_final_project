import { cleanskyinitiative_backend } from "../../declarations/cleanskyinitiative_backend";

const text_info = document.querySelector(".text_info");
const button_signup = document.querySelector(".button_signup");
const text_name = document.querySelector(".text_name");
const text_last_name = document.querySelector(".text_lastName");
button_signup.addEventListener("click", createUser);

async function createUser(e) {
  e.preventDefault();

  button_signup.setAttribute("disabled", true); // Butonu disabled yap
  const first_name = document.querySelector("#first_name").value.toString();
  const last_name = document.querySelector("#last_name").value.toString();
  const email = document.querySelector("#email").value.toString();

  await create_member(first_name, last_name, email);

  button_signup.removeAttribute("disabled"); // Butonu tekrar aktif yap

  return false;
}

async function create_member(first_name, last_name, email) {
  const api_data = await cleanskyinitiative_backend.create_member(
    first_name,
    last_name,
    email
  );
  if (api_data.Ok) {
    text_info.textContent = "User created successfully.";
    text_info.style.color = "greenyellow";
  } else if (api_data.Err.MissingBlank) {
    // text_info.textContent = "Aynı email yada boşluklardan birini doldurmadınız"
    text_info.textContent = api_data.Err.MissingBlank;
    text_info.style.color = "red";
  } else if (api_data.Err.SameEmail) {
    // text_info.textContent = "Aynı email yada boşluklardan birini doldurmadınız"
    text_info.textContent = api_data.Err.SameEmail;
    text_info.style.color = "red";
  } else if (api_data.Err.InvalidEmail) {
    // text_info.textContent = "Aynı email yada boşluklardan birini doldurmadınız"
    text_info.textContent = api_data.Err.InvalidEmail;
    text_info.style.color = "red";
  }

  console.log(api_data);
}

async function getUsers() {
  const users = await cleanskyinitiative_backend.list_members();
  console.log(users);
  const userList = document.querySelector(".user-list ul");
  userList.innerHTML = ""; // Önceki içeriği temizle

  // Kullanıcı verilerini alırken, her bir kullanıcı için ilgili HTML elementine yazdıralım
  users.forEach((user) => {
    const userListItem = document.createElement("li"); // Yeni bir <li> oluştur
    const userNameSpan = document.createElement("span"); // Yeni bir <span> oluştur
    const userLastNameSpan = document.createElement("span"); // Yeni bir <span> oluştur

    userNameSpan.textContent = user.name; // İlgili span'a adı yaz
    userNameSpan.classList.add("text_name"); // CSS sınıfını ekle

    userLastNameSpan.textContent = " " + user.lastname; // İlgili span'a boşluk ve soyadı yaz
    userLastNameSpan.classList.add("text_lastName"); // CSS sınıfını ekle

    userListItem.appendChild(userNameSpan); // <span> elementlerini <li> içine ekle
    userListItem.appendChild(userLastNameSpan); // <span> elementlerini <li> içine ekle

    userList.appendChild(userListItem); // <li> elementini <ul> içine ekle
  });
}

await getUsers();
