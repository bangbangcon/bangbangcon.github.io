---
layout: default-2021
title: Tickets - !!Con 2021
---

<style>
button{
    padding: 0.5rem 1rem;
    font-size: 1.2rem;
}
</style>

# !!Con 2021 Tickets

Hello! Tickets for !!Con 2021 will be released in two batches:

- Sat, April 24 at 10 a.m. Eastern
- Mon, April 26 at 10 p.m. Eastern

A link to our ticket site is below.

However, due to an <a href="https://twitter.com/bts_bighit/status/1248265665623908352">unfortunate naming collision</a>, we have to make sure everyone is aware:

## THIS IS <u>NOT</u> THE BTS CONCERT TICKET PAGE.

We are <u><strong>NOT</strong></u> affiliated with BTS or BANG.BANG.CON. You <u><strong>CANNOT</strong></u> buy tickets to the concert here.

<!--You cannot buy BTS concert tickets here. This page is for a tech conference in NYC.-->
<b>이 사이트에서는 BTS 방방콘 티켓을 구매하실 수 없습니다.</b> 이 사이트는 뉴욕시에서 열리는 테크 컨퍼런스를 위한 사이트입니다.

<hr>
The ticket you’re about to purchase is not for the BTS Concert, but for a tech conference. Would you like to purchase the tech conference ticket?

<!-- The ticket you’re about to purchase is not for the BTS Concert, but for a tech conference. Would you like to purchase the tech conference ticket? -->
지금 구매하시려는 티켓은 BTS 방방콘이 아닌 테크 컨퍼런스 티켓입니다. 테크 컨퍼런스 티켓을 구입하시겠습니까?

<div id="tixbtns" style="display:none">
<button id="bts">I want BTS Tickets instead!</button>
<!-- I want BTS Tickets instead! -->
<button id="btskr">BTS 방방콘 티켓을 구매하고 싶습니다!</button>
<div id="tixinfo">
<label>
<input type="checkbox" id="check">
I understand that I am buying tickets to the NYC tech conference and not the BTS concert.
</label>
</div> <!-- /tixinfo -->
</div> <!-- /tixbtns -->

<div id="tixlink">
You can purchase tickets to !!Con 2021 at <a href="https://bangbangcon.ticketspice.com/2021">bangbangcon.ticketspice.com/2021</a>
</div>

<div id="btstixkr" style="display:none">
<!-- BANG.BANG.CON is a free youtube concert. To watch, please visit the BANGTANTV channel on YouTube. You don’t need a ticket. -->
BTS 방방콘은 무료 유튜브 콘서트입니다. 시청하시려면 유튜브 채널 방탄TV를 확인해주세요. 티켓 없으셔도 됩니다.
</div>

<div id="btstix" style="display:none">
BANG.BANG.CON is a free youtube concert. To watch, please visit the BANGTANTV channel on YouTube. You don’t need a ticket.
</div>

<script>
document.getElementById('tixlink').style.display = "none";
document.getElementById('tixbtns').style.display = "block";
document.getElementById('check').onclick = function() {
  var s = document.getElementById('check').checked;
  document.getElementById('tixlink').style.display = s ? 'block' : 'none';
};

document.getElementById('bts').onclick = function() {
    document.getElementById('tixinfo').style.display = 'none';
    document.getElementById('btstixkr').style.display = 'none';
    document.getElementById('btstix').style.display = 'block';
    document.getElementById('tixlink').style.display = 'none';
}

document.getElementById('btskr').onclick = function() {
    document.getElementById('tixinfo').style.display = 'none';
    document.getElementById('btstixkr').style.display = 'block';
    document.getElementById('btstix').style.display = 'none';
    document.getElementById('tixlink').style.display = 'none';
}
</script>
