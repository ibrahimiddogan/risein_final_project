use candid::types::Type;
use ic_cdk::api;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,HttpResponse,
};
use serde_json::Value;
use candid::{CandidType, Decode, Deserialize, Encode, Nat};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::ptr::null;
use std::{borrow::Cow, cell::RefCell}; 

impl Storable for Member {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl Storable for Article {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
type Memory = VirtualMemory<DefaultMemoryImpl>;
const MAX_VALUE_SIZE: u32 = 100;
impl BoundedStorable for Article {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE; 
    const IS_FIXED_SIZE: bool = false;
}

impl BoundedStorable for Member {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE; 
    const IS_FIXED_SIZE: bool = false;
}
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static Member_Map: RefCell<StableBTreeMap<u64, Member, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|p| p.borrow().get(MemoryId::new(1))), 
        )
    );
    static Article_Map: RefCell<StableBTreeMap<u64, Article, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|p| p.borrow().get(MemoryId::new(1))), 
        )
    );
}


#[derive(CandidType, Deserialize,Clone)]
struct Member{
    name:String,
    lastname:String,
    email:String,
    registration_month: u32, 
}
#[derive(CandidType, Deserialize,Clone)]
struct Article {
    author_name: String,
    author_lastname: String,
    email: String,
    content: String,
}
#[derive(CandidType, Deserialize)]
enum ArticleErrors {
    MissingBlank(String),
    NotAllowed(String),
}

#[derive(CandidType, Deserialize)]
enum ArticleResult {
    Success(String),
    Error(ArticleErrors),
}

#[derive(CandidType, Deserialize)]
enum MemberErrors {
    MissingBlank(String)
}
#[derive(CandidType, Deserialize)]
enum Badgets {
    NewMember,
    ContributingMember,
    SeniorMember
}
#[derive(CandidType, Deserialize)]

enum MemberResult {
    Success(String),
    Error(MemberErrors),
}
#[ic_cdk::update]
fn create_member(name:String,lastname:String,email:String,registration_month:u32) ->  Result<MemberResult, MemberErrors> {
            if name.is_empty() || lastname.is_empty() || email.is_empty()  {
                return Err(MemberErrors::MissingBlank("İstenilen boşluklardan birini doldurmadınız".to_string()));
            }
            let _badge = match registration_month {
                0..=1 => Badgets::NewMember,
                2..=3 => Badgets::ContributingMember,
                _ => Badgets::SeniorMember,
            };
           Member_Map.with(|p|{
            let mut member_map = p.borrow_mut();
            let new_member = Member{
                name:name,
                lastname:lastname,
                email:email,
                registration_month:registration_month, 
            };
            let new_member_id=member_map.len();
        member_map.insert(new_member_id,new_member);
           });
           Ok(MemberResult::Success("Üyeliğiniz başarıyla oluşturuldu".to_string()))
 }

 #[ic_cdk::query]
fn list_members() -> Vec<Member> {
    let mut members = Vec::new();
    Member_Map.with(|p|{
        let member_map = p.borrow();
        for (_, member) in member_map.iter() {
            members.push(member.clone());
        }
    });
    members
}
#[ic_cdk::query]
fn get_new_members() -> Vec<Member> {
    let mut new_members = Vec::new();
    
    Member_Map.with(|p| {
        let member_map = p.borrow();
        for (_, member) in member_map.iter() {
            if member.registration_month <= 1 {
                new_members.push(member.clone());
            }
        }
    });
    
    new_members
}
#[ic_cdk::update]
fn publish_article(name: String, lastname: String, email: String, article_content: String) -> Result<String, String> {
    if name.is_empty() || lastname.is_empty() || email.is_empty() || article_content.is_empty() {
        return Err("Tüm alanları doldurmalısınız".to_string());
    }

    let mut member_found = false;

    Member_Map.with(|p| {
        let member_map = p.borrow();
        for (_, member) in member_map.iter() {
            if member.email == email {
                member_found = true;
                break;
            }
        }
    });

    if !member_found {
        return Err("Makale yayınlamak için kayıtlı bir üye olmalısınız".to_string());
    }

    
    let article = Article {
        author_name: name,
        author_lastname: lastname,
        email: email,
        content: article_content,
    };

   
    Ok("Makaleniz başarıyla yayınlandı".to_string())
}
#[ic_cdk::query]
fn get_article(name: String, lastname: String, email: String) -> Result<Article, String> {
    if name.is_empty() || lastname.is_empty() || email.is_empty() {
        return Err("Tüm alanları doldurmalısınız".to_string());
    }

    let mut member_found = false;
    let mut article_found = false;
    let mut article_content = String::new();

    Member_Map.with(|p| {
        let member_map = p.borrow();
        for (_, member) in member_map.iter() {
            if member.email == email {
                member_found = true;
                break;
            }
        }
    });

    if member_found {
        Article_Map.with(|p| {
            let article_map = p.borrow();
            for (_, article) in article_map.iter() {
                if article.email == email {
                    article_found = true;
                    article_content = article.content.clone();
                    break;
                }
            }
        });
    }

    if !member_found {
        return Err("Kullanıcı bulunamadı".to_string());
    }

    if !article_found {
        return Err("Makale bulunamadı".to_string());
    }

    Ok(Article {
        author_name: name,
        author_lastname: lastname,
        email: email,
        content: article_content,
    })
}
#[ic_cdk::update]
async fn get_events_from_api(city:String) -> String  {
    // Setup the URL for the HTTP GET request

    let api_key = "d9768701e8aca30a3ad653026ac052859a08e3687906e20b383a3ff045299625";
    let city = city;
    let url = format!("https://api.ambeedata.com/latest/by-city?city={}&x-api-key={}", city, api_key);
    let request_headers = vec![];
    
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers:request_headers ,
    };

    let response = match http_request(request, 1_603_096_000).await {
        Ok(response) => response.0,
        Err(_) => return "HTTP isteği başarısız oldu".to_string(), 
    };

    let json_response: Value = match serde_json::from_slice(&response.body) {
        Ok(value) => value,
        Err(_) => return "JSON dönüşümü başarısız".to_string(), 
    };

    let aqi = match json_response["stations"][0]["AQI"].as_f64() {
        Some(value) => value,
        None => return "AQI değeri bulunamadı".to_string(), 
    };
    format!("Şehrinizin hava indeksi={} (AQI)", aqi)
}






        

