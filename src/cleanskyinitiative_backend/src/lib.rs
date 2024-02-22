use candid::types::Type;
use ic_cdk::api;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,HttpResponse,
};
use serde_json::Value;
use candid::{CandidType, Decode, Deserialize, Encode, Nat};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
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
impl BoundedStorable for Article {
    const MAX_SIZE: u32 = 100; 
    const IS_FIXED_SIZE: bool = false;
}

impl BoundedStorable for Member {
    const MAX_SIZE: u32 = 100; 
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
            MEMORY_MANAGER.with(|p| p.borrow().get(MemoryId::new(2))), 
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
#[derive(CandidType, Deserialize,Clone,Debug)]
struct Article {
    author_name: String,
    email: String,
    tittle:String,
    content:String,
}
#[derive(CandidType, Deserialize)]
enum Errors {
    MissingBlank(String),
    SameEmail(String),
    NotAllowed(String),
    TittleNot(String),
}
#[derive(CandidType, Deserialize,Clone)]
enum Badgets {
    NewMember(String),
    ContributingMember(String),
    SeniorMember(String),
}
#[derive(CandidType, Deserialize)]
enum Ok {
    Success(String),
    Article(Article),
}
#[ic_cdk::update]
fn create_member(name:String,lastname:String,email:String) -> Result<Ok, Errors> {
            if name.is_empty() || lastname.is_empty() || email.is_empty()  {
                return Err(Errors::MissingBlank("İstenilen boşluklardan birini doldurmadınız".to_string()));
            }
            let email_exists = Member_Map.with(|p| {
                let member_map = p.borrow();
                member_map.iter().any(|(_, member)| member.email == email)
            });
            
            if email_exists {
                return Err(Errors::SameEmail("Girilen e-posta adresi zaten mevcut".to_string()));
            }           
           Member_Map.with(|p|{
            let mut member_map = p.borrow_mut();

            let registration_month=1;
            let badget = "New Member".to_string();
            
            let new_member = Member{
                name:name,
                lastname:lastname,
                email:email,
                registration_month:registration_month, 
            };
            
            let new_member_id=member_map.len();
            member_map.insert(new_member_id,new_member);
           });
           Ok(Ok::Success("Üyeliğiniz başarıyla oluşturuldu".to_string()))
 }

 #[ic_cdk::query]
fn list_members_name() -> Vec<(String)> {
    let mut members = Vec::new();
    Member_Map.with(|p|{
        let member_map = p.borrow();
        for (_, member) in member_map.iter() {
            members.push((member.name.clone()));
        }
    });
    members
}
#[ic_cdk::query]
fn list_members_lastname() -> Vec<(String)> {
    let mut members = Vec::new();
    Member_Map.with(|p|{
        let member_map = p.borrow();
        for (_, member) in member_map.iter() {
            members.push((member.lastname.clone()));
        }
    });
    members
}

#[ic_cdk::query]
fn get_new_members_name() -> Vec<(String)> {
    let mut new_members = Vec::new();
    
    Member_Map.with(|p| {
        let member_map = p.borrow();
        for (_, member) in member_map.iter() {
            if member.registration_month<=2 {
                new_members.push(member.name.clone());
            }
        }
    });
    
    new_members
}
#[ic_cdk::query]
fn get_new_members_lastname() -> Vec<(String)> {
    let mut new_members = Vec::new();
    
    Member_Map.with(|p| {
        let member_map = p.borrow();
        for (_, member) in member_map.iter() {
            if member.registration_month<=2{
                new_members.push(member.lastname.clone());
            }
        }
    });
    
    new_members
}


#[ic_cdk::update]
fn publish_article(name: String,email: String,tittle:String,content:String) -> Result<Ok, Errors> {
    if name.is_empty() || email.is_empty() ||  tittle.is_empty() {
        return Err(Errors::MissingBlank("Tüm alanları doldurmalısınız".to_string()));
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
        return Err(Errors::NotAllowed("Makale yayınlamak için kayıtlı bir üye olmalısınız".to_string()));
    }

    
    Article_Map.with(|p|{
        let mut article_map = p.borrow_mut();
        let new_article = Article{
            author_name:name,
            email:email,
            tittle:tittle,
            content:content,
        };
        
        let new_article_id=article_map.len();
        article_map.insert(new_article_id,new_article);
       });
    Ok(Ok::Success("Makaleniz başarıyla yayınlandı".to_string()))
}

#[ic_cdk::query]
fn get_article() -> Vec<String> {
    let mut new_article = Vec::new();
    
    Article_Map.with(|p| {
        let article_map = p.borrow();
        for (_, article) in article_map.iter() {
            
                new_article.push(article.content.clone());
        }
    });
    
    new_article
}

#[ic_cdk::update]
async fn get_events__city_from_api(city:String) -> String  {
    let api_key = "d9768701e8aca30a3ad653026ac052859a08e3687906e20b383a3ff045299625";
    let city = city;
    
    let url = format!("https://api.ambeedata.com/latest/by-city?city={}&x-api-key={}",city,api_key);
    let request_headers = vec![];
    
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers:request_headers ,
    };

    let response = match http_request(request, 1_603_112_400).await {
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
    format!("Şehrinizin hava kirliliği={} (AQI)", aqi)
}





        

