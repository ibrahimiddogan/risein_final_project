import { cleanskyinitiative_backend } from "../../declarations/cleanskyinitiative_backend";

const btn_publish = document.querySelector(".btn_publish");

btn_publish.addEventListener("click", sendMessage);

async function sendMessage(e) {
  e.preventDefault();

  btn_publish.setAttribute("disabled", true); // Butonu disabled yap

  const email = document.querySelector("#email").value.toString();
  const content = document.querySelector("#content").value.toString();

  await send_message(email, content);

  btn_publish.removeAttribute("disabled"); // Butonu tekrar aktif yap

  return false;
}

async function send_message(email, content) {
  const api_data = await cleanskyinitiative_backend.post_forum_message(
    email,
    content
  );
  console.log(api_data);
  if (api_data.Ok) {
    alert("Mesaj başarıyla gönderildi !");
  } else if (api_data.Err.MissingBlank) {
    // text_info.textContent = "Aynı email yada boşluklardan birini doldurmadınız"
    alert(api_data.Err.MissingBlank);
  } else if (api_data.Err.NotAllowed) {
    // text_info.textContent = "Aynı email yada boşluklardan birini doldurmadınız"
    alert(api_data.Err.NotAllowed);
  }
}

async function getMessages() {
  const api_data = await cleanskyinitiative_backend.get_forum_messages();
  console.log(api_data);

  const forumQuestions = document.querySelector(".forum__questions");

  // Her mesaj için bir question__wrapper oluştur ve içeriğini doldur
  api_data.forEach((message) => {
    const wrapper = document.createElement("div");
    wrapper.classList.add("question__wrapper");

    const content = document.createElement("p");
    content.classList.add("question__content");
    content.textContent = message.content;

    const email = document.createElement("p");
    email.classList.add("question_email");
    email.textContent = message.author;

    wrapper.appendChild(content);
    wrapper.appendChild(email);

    forumQuestions.appendChild(wrapper);
  });
}

getMessages();
