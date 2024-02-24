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
impl Storable for ForumMessage {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
type Memory = VirtualMemory<DefaultMemoryImpl>;
impl BoundedStorable for Article {
    const MAX_SIZE: u32 = 10000000; 
    const IS_FIXED_SIZE: bool = false;
}

impl BoundedStorable for Member {
    const MAX_SIZE: u32 = 10000000; 
    const IS_FIXED_SIZE: bool = false;
}
impl BoundedStorable for ForumMessage {
    const MAX_SIZE: u32 = 10000000; 
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
    static FORUM_MESSAGES: RefCell<StableBTreeMap<u64, ForumMessage, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|p| p.borrow().get(MemoryId::new(3))), 
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
    email: String,
    tittle:String,
    content:String,

}
#[derive(CandidType, Deserialize, Clone, Debug)]
struct ForumMessage {
    author: String,
    content: String,
}

#[derive(CandidType, Deserialize)]
enum Errors {
    MissingBlank(String),
    SameEmail(String),
    NotAllowed(String),
    TittleNot(String),
    InvalidEmail(String),
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
                return Err(Errors::MissingBlank("You haven't filled in all the required fields.".to_string()));
            }
            if !email.contains('@') {
                return Err(Errors::InvalidEmail("Invalid email address format.".to_string()));
            }
            let email_exists = Member_Map.with(|p| {
                let member_map = p.borrow();
                member_map.iter().any(|(_, member)| member.email == email)
            });
            
            if email_exists {
                return Err(Errors::SameEmail("The entered email address already exists.".to_string()));
            }           
            let registration_month=1;
            let _badget = "New Member".to_string();


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
           Ok(Ok::Success("Your membership has been successfully created.".to_string()))
 }

 #[ic_cdk::update]
 fn delete_member_by_email(email: String) -> Result<Ok, Errors> {
     if email.is_empty() {
         return Err(Errors::MissingBlank("E-posta adresi girilmedi".to_string()));
     }
     
     let mut member_found = false;
 
     Member_Map.with(|p| {
         let mut member_map = p.borrow_mut();
         let mut keys_to_remove = Vec::new();
 
         for (key, member) in member_map.iter() {
             if member.email == email {
                 keys_to_remove.push(key);
                 member_found = true;
             }
         }
 
         for key in keys_to_remove {
             member_map.remove(&key);
         }
     });
 
     if !member_found {
         return Err(Errors::NotAllowed("Girilen e-posta adresine sahip bir kullanıcı bulunamadı".to_string()));
     }
 
     Ok(Ok::Success("Kullanıcı başarıyla silindi".to_string()))
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
fn list_members_name() -> Vec<String> {
    let mut members = Vec::new();
    Member_Map.with(|p|{
        let member_map = p.borrow();
        for (_, member) in member_map.iter() {
            members.push(member.name.clone());
        }
    });
    members
}
#[ic_cdk::query]
fn list_members_lastname() -> Vec<String> {
    let mut members = Vec::new();
    Member_Map.with(|p|{
        let member_map = p.borrow();
        for (_, member) in member_map.iter() {
            members.push(member.lastname.clone());
        }
    });
    members
}
#[ic_cdk::query]
fn list_members_email() -> Vec<String> {
    let mut members = Vec::new();
    Member_Map.with(|p|{
        let member_map = p.borrow();
        for (_, member) in member_map.iter() {
            members.push(member.email.clone());
        }
    });
    members
}

#[ic_cdk::query]
fn get_new_members_name() -> Vec<String> {
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
fn get_new_members_lastname() -> Vec<String> {
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
fn publish_article(email: String,tittle:String,content:String) -> Result<Ok, Errors> {
    if content.is_empty() || email.is_empty() ||  tittle.is_empty() {
        return Err(Errors::MissingBlank("You must fill in all fields.".to_string()));
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
        return Err(Errors::NotAllowed("You must be a registered member to publish an article.".to_string()));
    }

    
    Article_Map.with(|p|{
        let mut article_map = p.borrow_mut();
        let new_article = Article{
            email:email,
            tittle:tittle,
            content:content,
        };
        
        let new_article_id=article_map.len();
        article_map.insert(new_article_id,new_article);
       });
    Ok(Ok::Success("Your article has been successfully published".to_string()))
}


#[ic_cdk::query]
fn get_article() -> Vec<Article> {
    let mut new_article = Vec::new();
    
    Article_Map.with(|p| {
        let  article_map = p.borrow();
        for (_, article) in article_map.iter() {
            new_article.push(article.clone());
        }
    });
    
    new_article
}



#[ic_cdk::update]
fn post_forum_message(user_email: String, message_content: String) -> Result<Ok, Errors> {
    if message_content.is_empty() {
        return Err(Errors::MissingBlank("Message content cannot be empty".to_string()));
    }

    let mut member_found = false;

    Member_Map.with(|p| {
        let member_map = p.borrow();
        for (_, member) in member_map.iter() {
            if member.email == user_email {
                member_found = true;
                break;
            }
        }
    });

    if !member_found {
        return Err(Errors::NotAllowed("You must be a registered member to send a message on the forum".to_string()));
    }

    let new_forum_message = ForumMessage {
        author: user_email.clone(),
        content: message_content.clone(),
    };
   
    // Forum mesajlarını saklamak için bir vektör kullanıyoruz
    FORUM_MESSAGES.with(|forum_messages| {
        let mut forum_messages = forum_messages.borrow_mut();
        let new_forum_id=forum_messages.len();
        forum_messages.insert(new_forum_id,new_forum_message);
    });

    Ok(Ok::Success("Your message has been successfully posted to the forum".to_string()))
}

#[ic_cdk::query]
fn get_forum_messages() -> Vec<ForumMessage> {
    let mut new_forum = Vec::new();
    
    FORUM_MESSAGES.with(|p| {
        let  forum_message = p.borrow();
        for (_, forum) in forum_message.iter() {
            new_forum.push(forum.clone());
        }
    });
    
    new_forum
}

#[ic_cdk::update]
async fn get_events_city_from_api(city:String) -> String  {
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
        Err(_) => return "HTTP request failed".to_string(), 
    };

    let json_response: Value = match serde_json::from_slice(&response.body) {
        Ok(value) => value,
        Err(_) => return "JSON conversion failed".to_string(), 
    };

    let aqi = match json_response["stations"][0]["AQI"].as_f64() {
        Some(value) => value,
        None => return "AQI value not found".to_string(), 
    };

    match aqi {
        0.0..=50.0 => format!("Air pollution in your city={} \n
        Air quality is good, no symptoms are present.  ", aqi),
        51.0..=100.0 => format!("Air pollution in your city={} \n
        Mild pollution, symptoms may occur in some sensitive individuals.  ", aqi),
        101.0..=151.0 => format!("Şehrinizin hava kirliliği={} \n
        Moderate pollution, symptoms may increase in sensitive individuals.  ", aqi),
        _ => format!("Air pollution in your city={} \n
        High pollution, symptoms may occur in the general population.  ", aqi),   
    }
}





        

