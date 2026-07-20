# Write your MySQL query statement below
with m as (select max(salary) maxSal, departmentId dId from Employee group by departmentId)
select d.name Department, e.name Employee, e.salary Salary
from Employee as e
inner join m on e.departmentId = m.dId and e.salary = m.maxSal
inner join Department as d on d.id = e.departmentId;