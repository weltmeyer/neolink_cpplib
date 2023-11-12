$env:OPENSSL_DIR = 'C:\Temp\vcpkg\installed\x64-windows-static'
$env:OPENSSL_STATIC = 'Yes'
[System.Environment]::SetEnvironmentVariable('OPENSSL_DIR', $env:OPENSSL_DIR, [System.EnvironmentVariableTarget]::User)
[System.Environment]::SetEnvironmentVariable('OPENSSL_STATIC', $env:OPENSSL_STATIC, [System.EnvironmentVariableTarget]::User)

$env:PROTOC="C:\Temp\protoc-25.0-win64\bin\protoc.exe"

$env:path=$env:path+";C:\msys64\mingw64\bin"