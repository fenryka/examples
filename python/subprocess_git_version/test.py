import subprocess
output = subprocess.run(["which", "git"], capture_output=True, text=True)
print (f"out = '{output.stdout}'', err = '{output.stderr}'")
output = subprocess.run(["which", "gitt"], capture_output=True, text=True)
print (f"out = '{output.stdout}', {len(output.stdout)}, err = '{output.stderr}'")
output = subprocess.run(["git", "--version"], capture_output=True, text=True)
print(output.stdout)


print(output.stderr)
