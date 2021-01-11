rem set defaults
set RUST_LOG=info
set RUST_LOG_STYLE=always

rem set all environment variables
set ems_url=tcp://localhost:7222
set ems_user=testuser
set ems_password=testpw
set ems_input_dest_name=mytopic
set ems_input_dest_type=topic

tibco-ems-sender-rs.exe