drop table if exists oauth_client;

create table oauth_client (
    id text primary key,
    name text not null,
    login_url text not null,
    userinfo_url text not null
);

insert into oauth_client (id, name, login_url, userinfo_url) values ('52370d89370ed2bdddc4ff76c6e019ebcc6ca8b6fe84e15b7cd053ef19929ee7', 'Whodis' , 'http://localhost:5173/login', 'http://localhost:8001/userinfo' );
