- add

```sql
  CREATE TABLE <table_name> (
    <column_name> <TYPE>
  );
```

- view
  `SHOW TABLES;`

- change

```sql
  ALTER TABLE <name> ADD <column_name> <TYPE>;
  ALTER TABLE <name> MODIFY <column_name> <TYPE>;
```

- list contents of table
  `SELECT * FROM <table_name>`

- list specification of table

```sql
  select COLUMN_NAME, DATA_TYPE, CHARACTER_MAXIMUM_LENGTH,
         NUMERIC_PRECISION, DATETIME_PRECISION,
         IS_NULLABLE
  from INFORMATION_SCHEMA.COLUMNS
  where TABLE_NAME= '<name>';
```

- id (dependent table example)

```sql
  CREATE TABLE <name> (
    <id> INT NOT NULL AUTO_INCREMENT,
    <parent_id> INT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (<parent_id>) REFERENCES <parent_table>(<id>)
  );
```
