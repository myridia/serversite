# serversite
Basic static HTLM Website to present a Server

## Usage
We have many servers online what are performing public services like a Mail Transfer Agent delivery, in short called MTA.
Often other Companies or Institution require us to identify those Server, so we create a basic HTML Website to show under the main server Domain
Example: https://grallator.com

## What to show
* Data pulled from json data file
* Main Services with entrance
* Legal Info

## Requirement
```
apt-get install npm
```

## Install
```
git clone https://github.com/myridia.com/serversite.git
cd serversite/themes/bootstrap/
npm install
```

## Setup Index forwarder
```
ln -s serversite/builder/index.html  index.html
```


