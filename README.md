# axum-template
my axum template

```shell
# 登录控制台
psql -h 127.0.0.1 -p 5432 -d postgres -U $(whoami)
# 创建数据库用户
CREATE USER postgres WITH PASSWORD '123456';
# 将数据库所有权限赋予postgres用户
CREATE DATABASE postgres OWNER postgres;
# 将数据库所有权限赋予postgres用户
GRANT ALL PRIVILEGES ON DATABASE postgres to postgres;
# 用户管理数据库角色
ALTER ROLE postgres CREATEDB;
```

```shell
mkdir -p ~/PL/db/data/pg_data
pg_ctl init -D ~/PL/db/data/pg_data
pg_ctl start -D ~/PL/db/data/pg_data -l ~/pl/db/data/log.log
export DATABASE_URL="postgres://postgres:123456@localhost/todos"
```
