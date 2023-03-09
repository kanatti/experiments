alias build='docker build -t experiments .'
alias fsfs='docker run experiments /app/fsfs'
alias rmc='docker rm $(docker ps -a -q)'