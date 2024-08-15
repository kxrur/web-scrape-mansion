- insert

```sql
  INSERT INTO <table_name> (<column1_name>, <column2_name)
  VALUES (<data1>, <data2>), (<data1>, <data2>);
```

- delete

```sql
  DELETE FROM <table> WHERE <column_name> = <condition>;
```

- query

```sql
  SELECT * FROM <table_name>;
```

- query column(s)

```sql
  SELECT <column_name>, <column2_name> FROM <table_name>;
```

- update

```sql
  UPDATE <table_name>
  SET <column_name> = <data>
  -- without this line all the values in the column are changed to <data>
  WHERE <column_name> = <condition>;
```

- where condition

```sql
  WHERE <data> LIKE '%ER%'; -- matches jioERijo
  WHERE <data> > 1000 OR <data> < 100;
  WHERE <data> < 1000 AND <data> > 100;
  WHERE <data> BETWEEN 100 AND 1000;
  WHERE <data> IS NUL;

```

- inner join

```sql
  SELECT * FROM <table1>
  JOIN <table2> ON <table1.column_name> = <table2.column_name>;
```

- left join (list all left table's items + matches on right)
- right join (list all right table's items + matches on left)
