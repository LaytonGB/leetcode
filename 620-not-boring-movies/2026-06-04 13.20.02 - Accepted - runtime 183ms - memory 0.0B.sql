-- Write your PostgreSQL query statement below
select * from Cinema as c
where id % 2 = 1 and description <> 'boring'
order by rating desc;