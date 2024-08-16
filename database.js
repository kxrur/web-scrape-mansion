import mysql from 'mysql2';
import dotenv from 'dotenv'
dotenv.config()

const pool = mysql.createPool({
  // values are set in .env, it is not pushed to GH 
  host: process.env.MYSQL_HOST,
  user: process.env.MYSQL_USER,
  password: process.env.MYSQL_PASSWORD,
  database: process.env.MYSQL_DATABASE
}).promise();


async function getPictures() {
  const result = await pool.query("SELECT * FROM pictures");
  return result[0]
}

async function getPicture(id) {
  const result = await pool.query(`
    SELECT * FROM pictures
    WHERE id = ?
    `, [id]);

  //WHERE id = ${id} is bad cuz id is an 'untrusted value'
  //result in security leak b/c of HTTP request stuff
  return result[0]
}

async function createMansion(address1) {
  const [result] = await pool.query(`
  INSERT INTO mansions (address1)
  VALUES (?)
  `, [address1])
  return result.insertId
}

async function createPicture(name, mansion_id) {
  await pool.query(`
  INSERT INTO pictures (name, mansion_id)
  VALUES (?, ?)
  `, [name, mansion_id])
}

const rows = await getPictures();
const first_row = await getPicture(1);
for (let i = 2; i <= 99; i++) {
  await createMansion(i)
}
await createPicture("pic999", 99);
const all_pics = await getPictures();
console.log(first_row)
console.log(all_pics)
