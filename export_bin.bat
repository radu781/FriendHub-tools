@echo off

if not exist "..\FriendHub\bin" {
    mkdir "..\FriendHub\bin"
}

if not exist "..\FriendHub\bin\email" {
    mkdir "..\FriendHub\bin\email"
}

copy /Y "email\target\debug\email.exe" "..\FriendHub\bin\email\email.exe"
copy /Y "email\.env" "..\FriendHub\bin\email\.env"
