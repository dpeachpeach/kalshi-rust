curl --request POST \
     --url https://demo-api.kalshi.co/trade-api/v2/portfolio/orders \
     --header 'Authorization: {token}' \
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