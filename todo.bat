@echo off
echo -------
echo -------
echo ==TODOS:
findstr -s -n -i -l "TODO" src/*.rs
findstr -s -n -i -l "TODO" examples/*.rs
findstr -s -n -i -l "TODO" tests/*.rs
findstr -s -n -i -l "TODO" benchmarks/*.rs
echo -------
echo -------
