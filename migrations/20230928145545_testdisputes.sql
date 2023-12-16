CREATE TABLE IF NOT EXISTS test_disputes (
  id varchar(36) primary key not null,
  order_id varchar(36) unique not null
);