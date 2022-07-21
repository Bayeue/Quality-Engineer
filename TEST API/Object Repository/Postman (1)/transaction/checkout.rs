<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>checkout</name>
   <tag></tag>
   <elementGuidId>57c00d70-53ab-4dad-bcd5-a9ee661af696</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;id_customer&quot;,
      &quot;value&quot;: &quot;0885631941223&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;customer_name&quot;,
      &quot;value&quot;: &quot;Maura&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-callback-token</name>
      <type>Main</type>
      <value>BjVVRO8eKgceve38jmqm6twtK9YLjtAfk7CbJLxfiToTilHX</value>
      <webElementGuid>a4e99f62-ab39-48e4-a28f-1a708fcd3eb0</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${baseurl}/users/checkout/pulsa-axis-50rb</restUrl>
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
      <id>bb7ae600-0271-437b-b054-4816bebee29b</id>
      <masked>false</masked>
      <name>baseurl</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
