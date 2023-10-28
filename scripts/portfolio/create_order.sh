curl --request POST \
     --url https://demo-api.kalshi.co/trade-api/v2/portfolio/orders \
     --header 'Authorization: Bearer 47c0ab93-159a-47dc-b17b-76c9067aefe4:b1PgqR3wPjiY7LrBeg9xs3ohqFqAPhu248iXRb8ZX8kJau3JQyXTIVHQO09bkjGS' \
     --header 'accept: application/json' \
     --header 'content-type: application/json' \
     --data '
{
  "action": "buy",
  "ticker": "GOVSHUTLENGTH-23DEC31-T14",
  "client_order_id": "2b2781e1-6ff8-4e35-87b4-fa8fd0a81fd4",
  "count": 1,
  "side": "yes",
  "type": "market",
  "buy_max_cost": null 
}
'