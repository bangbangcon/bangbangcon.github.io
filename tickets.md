---
layout: default-2020
title: Tickets - !!Con 2020
---

# BANGBANGCON

## THIS IS NOT THE BTS CONCERT TICKET PAGE.

We are <strong>NOT</strong> affiliated with BTS or BANG.BANG.CON. You <strong>cannot</strong> buy tickets to the concert here.

<label>
<input type="checkbox" id="check">
I understand that I am buying tickets to the NYC tech conference and not the BTS concert.
</label>

<div id="tixlink">

You can purchase tickets to !!Con 2020 at <a href=""></a>

</div>

<script>
document.getElementById('tixlink').style.display = "None"
document.getElementById('check').onclick = function() {
  var s = document.getElementById('check').checked;
  document.getElementById('tixlink').style.display = s ? 'block' : 'none';
};
</script>
