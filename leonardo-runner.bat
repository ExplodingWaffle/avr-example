:: start /B putty.exe -serial com6 -sercfg 1200,8,n,1,N -N
:: sleep 1
:: taskkill /im putty.exe /f
:: sleep 1
pushd
cd %~dp1
avrdude -C"C:\Program Files (x86)\Arduino\hardware\tools\avr/etc/avrdude.conf" -vqq -patmega32u4 -cavr109 -PCOM7 -b57600 -D "-Uflash:w:%~nx1:e"
popd
exit /B 0