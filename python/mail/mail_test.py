#!/usr/bin/python

import smtplib

#
# SMTP: <some mail host>
# SMTP port: <mail port>
# Connection sec: STARTTLS
#

sender = 'sender@thing.com'
receivers = ['person1@some.thing.com', 'person2@something.com' ]

message = """From: Knowlege Harvesting <sender@thing.com>
To: All the squishy humans
Subject: Field Eng: Knowledge Harvesting test 

This is a test e-mail message, enjoy it :)
"""

try:
    s = smtplib.SMTP ('some.mailhost.com')
    s.set_debuglevel(True)
    s.starttls()
    s.ehlo()
    s.login ('<user>', '<password>')
    s.sendmail (sender, receivers, message)
    s.quit()

    print "Successfully sent email"


except smtplib.SMTPException as e :
    print "Error: unable to send email"
    print e

