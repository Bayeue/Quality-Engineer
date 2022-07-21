<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>callback</name>
   <tag></tag>
   <elementGuidId>96835913-d5c2-4973-8c61-a5e5217cb3ff</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;id\&quot;: \&quot;62bb210af01c3236811fc564\&quot;,\r\n    \&quot;status\&quot;: \&quot;PAID\&quot;,\r\n    \&quot;paid_amount\&quot;: 2000000,\r\n    \&quot;payment_method\&quot;: \&quot;PAYMENT_BANK\&quot;,\r\n    \&quot;paid_at\&quot;: \&quot;2020-01-14T02:32:50.912Z\&quot;,\r\n    \&quot;payment_channel\&quot;: \&quot;MANDIRI\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${baseurl}/transaction/callback_invoice</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseurl</defaultValue>
      <description></description>
      <id>e8c0177b-5f02-4db7-ba6f-51fc38b02ced</id>
      <masked>false</masked>
      <name>baseurl</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
