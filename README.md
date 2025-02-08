### Example Payment Gateway Axum + Seaorm





### Cara Kerjanya Payment Gateway ini yang menggunakan axum + seaorm
<img src="./images/payment_gateway.png" alt="payment" />


### OpenApi in Utoipa
<img src="./images/example.png" alt="openapi" />


### Migrate

```sh
sea-orm-cli migrate init
```

### Generate Entity

```sh
sea-orm-cli generate entity -o src/entities
```

### Migrate run UP

```sh
sea-orm-cli migrate up
```

### Migrate run Down

```sh
sea-orm-cli migrate down
```
