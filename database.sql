drop table if exists exposure_sites;
create table exposure_sites
(
   id serial primary key,
   exposure_id varchar(140) not null,
   place_name varchar(140) not null,
   address_line1 varchar(140) not null,
   suburb varchar(140) not null,
   postcode varchar(140) not null,
   exposure_time_from TIMESTAMP not null,
   exposure_time_to TIMESTAMP not null,
   posted_time TIMESTAMP default now() not null
);

/* Load seed data for testing */
-- insert into exposure_sites
--    (exposure_id,
--     place_name,
--     address_line1,
--     suburb, 
--     exposure_time_from, 
--     exposure_time_to, 
--     posted_time
--     )
-- values(1, 'Woolworths', 'KenmorePlaza', 'Kenmore','2022-01-01 05:40:00', '2022-01-01 07:40:00', '2022-01-03 05:40:00');
-- insert into exposure_sites
--    (exposure_id,
--     place_name, 
--     address_line1,
--     suburb, 
--     exposure_time_from, 
--     exposure_time_to, 
--     posted_time)
-- values(2, 'Post Office', 'Kenmore Plaza', 'Kenmore','2022-01-02 05:40:00', '2022-01-02 07:40:00', '2022-01-04 05:40:00');