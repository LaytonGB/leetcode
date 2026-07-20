-- Write your PostgreSQL query statement below
select class from (
    select class, count(*) count from Courses
    group by class
) where count >= 5;