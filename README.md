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
cd ../../../../
ln -s serversite/builder/index.html  index.html
```

## Make phpMyAdmin accessible via Froxlor on a custom domain

1. In Froxlor, go to **Domains → Edit your domain** (e.g. `gms.salamander-jewelry.net`)
2. Click the **SSL** tab
3. Scroll down to **IPs and Ports**, then click the IP/port entry
4. Scroll to **Own SSL vHost-settings**

> The content of this field will be included into this ip/port vHost container directly. You can use the following variables:
> `{DOMAIN}`, `{DOCROOT}`, `{CUSTOMER}`, `{IP}`, `{PORT}`, `{SCHEME}`, `{FPMSOCKET}` (if applicable)
>
> **Attention:** The code won't be checked for any errors. If it contains errors, webserver might not start again!

5. Paste the following (Nginx example):

```
location /phpmyadmin {
    alias /usr/share/phpmyadmin/;
    index index.php;

    location ~ \.php$ {
        fastcgi_split_path_info ^(.+?\.php)(/.*)$;
        include /etc/nginx/fastcgi_params;
        fastcgi_param SCRIPT_FILENAME $request_filename;
        try_files $fastcgi_script_name =404;
        set $path_info $fastcgi_path_info;
        fastcgi_param PATH_INFO $path_info;
        fastcgi_pass unix:/var/run/php/1-froxlor.panel-web.salamander-jewelry.net-php-fpm.socket;
        fastcgi_index index.php;
    }

    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff2?|ttf|eot)$ {
        expires max;
        log_not_found off;
    }
}
```

6. Save — the domain will now serve phpMyAdmin at `https://your-domain/phpmyadmin/`.


