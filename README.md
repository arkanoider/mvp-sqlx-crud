# mvp-sqlx-crud
Minimum Viable Product test create sqlx-crud

This is small MVP to ask an advice on what is going wrong here, basically we are just using this sql table:
```
CREATE TABLE IF NOT EXISTS test_disputes (
  id varchar(36) primary key not null,
  order_id varchar(36) unique not null
);
```

to add with [sqlx-crud](https://crates.io/crates/sqlx-crud) crate an entry with ```create``` method.
Two field are Uuid and the result that we have calling the test function with:

```cargo test test_new_dispute_fake```

is 
```
running 1 test
test tests::test_new_dispute_fake ... FAILED

failures:

---- tests::test_new_dispute_fake stdout ----
TestDispute { id: 45b2294d-fafc-4043-87c7-4127ef9dbea4, order_id: a72b809d-3894-4579-9d2f-147493d3df19 }
Error: error returned from database: (code: 1299) NOT NULL constraint failed: test_disputes.id

Caused by:
    (code: 1299) NOT NULL constraint failed: test_disputes.id
```

I cannot understand what I am missing, any advice? Probably a super noob error, but I have no idea on the problem.

Thanks!




