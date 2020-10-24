if not exist "..\..\target" mkdir "..\..\target"

for %%t in (debug,release) do (
    if not exist "..\..\target\%%t" mkdir "..\..\target\%%t"
    copy /Y copy\* ..\..\target\%%t\
    if exist "..\..\target\%%t\resources" rmdir ..\..\target\%%t\resources
    mklink /J ..\..\target\%%t\resources resources
)