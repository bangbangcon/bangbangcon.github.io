---
layout: default-2021
title: Program - !!Con 2021
---

# Conference program

<style type="text/css">
    table {
        table-layout: fixed;
        width: 100%;
        border-collapse: collapse;

    }
    tr:nth-child(even){
        background-color: white;
    }
    td {
        padding: 0.25rem;
        border-color: white;
    }
    table, tr, td, th {
        border: 1px solid #8788A8;
    }
    .syncwatch {
        background-color: #D0D2F5;
    }
    .session {
        background-color: #F5D0F2;
    }
    .unconf {
        background-color: #8788A8;
        color: white;
    }
    .keynote {
        background-color: #8788A8;
        color: white;
    }
    th.current {
        background-color: #A887A6;
        color: white;
    }
    .current {
        font-weight: bold;
    }
</style>

Timezone: <span id="timezone">Pacific Daylight Time (GMT -7)</span>.

<table>
    <tr>
        <th></th>
        <th>Sat 15<sup>th</sup></th>
        <th>Sun 16<sup>th</sup></th>
        <th>Mon 17<sup>th</sup></th>
        <th>Tue 18<sup>th</sup></th>
        <th>Wed 19<sup>th</sup></th>
        <th>Thu 20<sup>th</sup></th>
        <th>Fri 21<sup>st</sup></th>
        <th>Sat 22<sup>nd</sup></th>
    </tr>
    <tr>
        <th><time datetime="9:00">9:00</time></th>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td class="syncwatch">Sync 8</td>
    </tr>
    <tr>
        <th><time datetime="10:00">10:00</time></th>
        <td></td>
        <td class="syncwatch">Sync 1</td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td class="session">Session 9</td>
    </tr>
    <tr>
        <th><time datetime="11:00">11:00</time></th>
        <td></td>
        <td class="syncwatch">Sync 2</td>
        <td class="syncwatch">Sync 3</td>
        <td class="syncwatch">Sync 4</td>
        <td class="session">Session 6</td>
        <td class="syncwatch">Sync 5</td>
        <td class="syncwatch">Sync 7</td>
        <td class="keynote">Keynote</td>
    </tr>
    <tr>
        <th><time datetime="12:00">12:00</time></th>
        <td class="keynote">Keynote</td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
    </tr>
    <tr>
        <th><time datetime="13:00">13:00</time></th>
        <td class="session">Session 1</td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
    </tr>
    <tr>
        <th><time datetime="14:00">14:00</time></th>
        <td class="unconf">Unconf.</td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
    </tr>
    <tr>
        <th><time datetime="15:00">15:00</time></th>
        <td class="session">Session 2</td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
    </tr>
    <tr>
        <th><time datetime="16:00">16:00</time></th>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
    </tr>
    <tr>
        <th><time datetime="17:00">17:00</time></th>
        <td></td>
        <td class="session">Session 3</td>
        <td class="session">Session 4</td>
        <td class="session">Session 5</td>
        <td class="syncwatch">Sync 6</td>
        <td class="session">Session 7</td>
        <td class="session">Session 8</td>
        <td class="syncwatch">Sync 9</td>
    </tr>
</table>

<script>
    // Get tz as a string, e.g. "Eastern Daylight Time"
    const dateStr = new Date().toString();
    const timezone = dateStr.substring(dateStr.indexOf('(') + 1, dateStr.length - 1);
    document.getElementById('timezone').innerHTML = timezone + ' (detected)';

    // Replace time elements with tz-adjusted hours.
    Array.from(document.getElementsByTagName('time')).forEach(timeTag => {
        const d = new Date(`May 1, 2020 ${timeTag.getAttribute('datetime')}:00 GMT-0700`);
        timeTag.innerHTML = `${d.getHours()}:00`;
    });

    // Highlight current day (getDate gets day of month!)
    const currentDay = new Date().getDate();
    // getMonth is 0-based! May == 4.
    if (new Date().getMonth() == 4 || true) {
        if (currentDay >= 15 && currentDay <= 22 || true) {
            const rowToHighlight = (currentDay - 15) + 1;
            Array.from(document.getElementsByTagName('tr')).forEach(row => {
                row.children.item(rowToHighlight).classList.add('current');
            })
        }
    }
    console.log(currentDay);
</script>
