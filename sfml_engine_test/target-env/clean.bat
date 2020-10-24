for %%t in (debug,release) do (
    if exist "..\..\target\%%t\resources" rmdir ..\..\target\%%t\resources
)

cargo clean

repair.bat