function generate(response) {
    var positions = response.positions;
    var position_transactions = response.position_transactions;
    var prices = response.position_prices;
    var transactions = response.transactions;

    convert_transactions(transactions, position_transactions);

    chart(document.getElementById("chart_value"), 'line', 'Value', value(positions, position_transactions, prices));
    chart(document.getElementById("chart_cash_flow"), 'scatter', 'Cash Flow', cash_flow(transactions));
    chart(document.getElementById("chart_net_cash_flow"), 'line', 'Net Cash Flow', net_cash_flow(transactions));
}

function onload() {
    download(function(response) {
        generate(response);
    });
}