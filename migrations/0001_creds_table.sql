create table creds (
    usr varchar null,
    pass varchar null,
    website varchar null
);

create unique index cred_usr_lookup on creds (usr);
