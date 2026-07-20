-- Write your PostgreSQL query statement below
with x as (
    select customer_number, count(customer_number) count from Orders
    group by customer_number
    order by count desc
    limit 1
) select x.customer_number from x;