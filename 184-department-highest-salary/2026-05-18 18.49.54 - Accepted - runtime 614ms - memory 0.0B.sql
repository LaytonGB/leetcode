with w as (
    select name, salary, departmentId,
    # rank generates an id in the order of the `over` content
    rank() over(
        # partitioning tells sql what to group by when ranking
        partition by departmentId
        order by salary desc
    ) as r
    from Employee
)
select d.name Department, w.name Employee, w.salary Salary
from w
# only keep employees with top ranks (highest salary)
inner join Department d on d.id = w.departmentId
where w.r = 1;