# bipa-backend-challenge
Back-end coding challenge for Bipa intership

## Build tools & versions used
Docker, Docker compose, Rust 1.8.7
## Steps to run the app
docker compose up
## What was the reason for your focus? What problems were you trying to solve?
I was trying to fetch a list of jsons from an api periodcally, apply some transformations, store it in a database and sending the data as a response to a get request at /nodes endpoint. The reason of my focus was handling error cases such as losing connection to db.

## How long did you spend on this project?
1.5 days
## Did you make any trade-offs for this project? What would you have done differently with more time?
Due to the time constraint, I hard coded configurations parameters such as ports, hostname, api link and didn't use environmental variables. If I had more time, I would use envs to be able to change some settings without having to recompile. I also wouldn't store credentials in plain text.
## What do you think is the weakest part of your project?
The error handling could be more through and there is no error handling for poisoned mutexes, although I don't think it's possible to occur.
## Is there any other information you’d like us to know?
