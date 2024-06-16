create table vaults (
    nam varchar,
    pass varchar
);

create unique index vault_nam_lookup on vaults(nam);

/*
create table <vaultname>_vault (
    usr varchar,
    pass varchar,
    service varchar,
)
*/
