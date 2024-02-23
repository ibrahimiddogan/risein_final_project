import { cleanskyinitiative_backend } from "../../declarations/cleanskyinitiative_backend";

const title = document.querySelector("#title");
const email = document.querySelector("#email");
const content = document.querySelector("#content");

const btn_publish = document.querySelector(".btn_publish");

const text_info = document.querySelector(".text_info");

btn_publish.addEventListener("click", publishArticle);

async function publishArticle(e) {
  e.preventDefault();

  btn_publish.setAttribute("disabled", true); // Butonu disabled yap
  const title = document.querySelector("#title").value.toString();
  const email = document.querySelector("#email").value.toString();
  const content = document.querySelector("#content").value.toString();

  await publish_article(email, title, content);

  btn_publish.removeAttribute("disabled"); // Butonu tekrar aktif yap

  return false;
}

async function publish_article(email, title, content) {
  const api_data = await cleanskyinitiative_backend.publish_article(
    email,
    title,
    content
  );
  if (api_data.Ok) {
    text_info.textContent = "Makale başarıyla yayınlandı !";
    text_info.style.color = "greenyellow";
  } else if (api_data.Err.MissingBlank) {
    // text_info.textContent = "Aynı email yada boşluklardan birini doldurmadınız"
    text_info.textContent = api_data.Err.MissingBlank;
    text_info.style.color = "red";
  } else if (api_data.Err.NotAllowed) {
    // text_info.textContent = "Aynı email yada boşluklardan birini doldurmadınız"
    text_info.textContent = api_data.Err.NotAllowed;
    text_info.style.color = "red";
  }
}

async function getArticles() {
  const articles = await cleanskyinitiative_backend.get_article();
  console.log(articles);

  const articlesList = document.getElementById("articlesList");

  // Önce var olan makaleleri temizle
  articlesList.innerHTML = "";

  // Her makale için bir <li> elemanı oluştur ve içeriğini doldur
  articles.forEach((article) => {
    const li = document.createElement("li");
    li.classList.add("articleItem");

    const title = document.createElement("p");
    title.classList.add("articleTitle");
    title.textContent = article.tittle;

    const content = document.createElement("p");
    content.classList.add("articleContent");
    content.textContent = article.content;

    li.appendChild(title);
    li.appendChild(content);

    articlesList.appendChild(li);
  });
}

await getArticles();
