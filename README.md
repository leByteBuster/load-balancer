# Start Simple Backend Mock-Servers  
- open python-mock-backend in a terminal
- python3 -m http.server 5000
- python3 -m http.server 5001
    (currently server localhost:5000, localhost:5001 are hardcoded) 

# Start application 
- `cargo run` serves the load balancer at 8080. calling it should alternate between responses from 5000 and 5001
