---
layout: default-2022
title: Program - !!Con 2022
---

# Conference Program

<style type="text/css">
    /* Timezone */
    #timezone {
        font-family: monospace;
    }
    ol {
        font-size: 20px;
    }
</style>

<!-- Extra nbsp to make it look good :) -->

Timezone: &nbsp;<span id="timezone">Pacific Daylight Time (GMT -7)</span>.

All times in 24h format.

## Schedule 

<span id="cards-section"></span>

### Saturday, Nov 12

<div class="scheduletable">

| Time            | Event
|-----------------|-------------------------------------------------------------------------
| <time datetime="12:00">12:00</time>        | First Item
| <time datetime="13:00">13:00</time>        | Second Item

</div>

### Sunday, Nov 13

<div class="scheduletable">

| Time            | Event
|-----------------|-------------------------------------------------------------------------
| <time datetime="12:00">12:00</time>        | First Item
| <time datetime="13:00">13:00</time>        | Second Item

</div>

<script>
    // Get tz as a string, e.g. "Eastern Daylight Time".
    const dateStr = new Date().toString();
    const timezone = dateStr.substring(dateStr.indexOf('(') + 1, dateStr.length - 1);
    document.getElementById('timezone').innerHTML = timezone + ' (detected)';

    // Replace time elements with tz-adjusted hours.
    Array.from(document.getElementsByTagName('time')).forEach(timeTag => {
        const d = new Date(`May 1, 2020 ${timeTag.getAttribute('datetime')}:00 GMT-0700`);
        const minutes = `${d.getMinutes()}`.padStart(2, '0');
        timeTag.innerHTML = `${d.getHours()}:${minutes}`;
    });
</script>
