




pub enum PermisionFromUsers{
    ADM,
    NORMAL
}

pub struct UserEntity{

    name:String,
    permision:PermisionFromUsers,
    email:String,
    password:String
}


impl UserEntity{
    pub fn is_email_valid(email:&str)->bool{
        if email.len() == 0{
            return false
        }

        if email.split("@").collect::<Vec<&str>>().len() == 1{
            return false
        };

        if !(email.contains(".com") || email.contains(".com.br")){
            return false
        };
        
        true
    }

    pub fn is_name_valid(name:&str)->bool{
        if name.len() < 4 || name.len() >100{
            return false;
        };
        true
    }

    pub fn new(user:UserEntity) -> Result<UserEntity,&'static str>{
        let email_validate = UserEntity::is_email_valid(&user.email);
        if !email_validate{
            return Err("Email is invalid!");
        };

        let name_validate = UserEntity::is_name_valid(&user.name);
        if !name_validate{
            return Err("Name is invalid");
        }

        return Ok(user);
    }

}