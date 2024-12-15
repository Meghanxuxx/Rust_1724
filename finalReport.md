# Final Report

**Team member**: Huilin Xu (1005969099, meghan.xu@mail.utoronto.ca), Shanzhi Zhang (,), Xian Wu (1009735146, amandaxian.wu@mail.utoronto.ca)

## Motivation

As international students and new immigrants, we recognize the challenges newcomers had in finding employment and the urgent need to address this gap. Currently, many new immigrants, students, and marginalized groups are facing some significant disadvantages in finding jobs, not only in Canada, but all around the world. We recently surveyed 200 people and discovered six major obstacles that are preventing them from reaching their employment aspirations:

1. **Language Barriers**: Many non-native speakers find it challenging to create resumes and cover letters that capture the subtleties of the local language and meet professional expectations.
2. **Professional Quality Standards**: Having high-quality, polished, and professional job application materials is essential for finding a job successfully, but it can sometimes feel out of reach for those who don’t have guidance or access to professional services.
3. **Limited Network Access**: Many immigrants find themselves without connections to local professionals in their field, which makes it challenging to access culturally relevant and industry-specific mentorship, results in professional services becoming unaffordable.
4. **Frequent Revisions Needed**: Job application materials, like resumes and cover letters, typically require several rounds of editing. However, mentors and advisors often can't provide the necessary extensive, ongoing support.
5. **Personalization challenges**: Creating application materials requires personal insight, making it hard for individuals to rely only on outside assistance to produce effective documents.
6. **Contextual Barriers**: Being unfamiliar with local industry standards, such as those in Canada, can impact one’s preparedness and potentially lower the success rates in job applications.

These barriers to job searching, caused by limited resources and a lack of knowledge about the position or company, further affect the two most important documents in the application process—the resume and the cover letter. Our project chose to focus on cover letters, partly due to the short length of the program, but more importantly because applicants often overlook the significance of cover letters or struggle to meet expectations. While cover letters are optional in the job application process for some companies, there are still a significant number of positions that require applicants to submit them as an opportunity to demonstrate their understanding of the position and the organization, career experience, and ambition to help the applicant stand out from the pool of candidates. Writing an effective cover letter can be challenging and requires:

1. Overview of job role and responsibilities
2. Alignment with the company's culture and mission
3. Relevant professional experience matching the job description
4. Strong language proficiency
5. Genuine enthusiasm for the role
6. Demonstrated future potential

Therefore combining our own needs as well as those of our target group, the goal of our project is to develop an AI-powered platform designed to increase the competitiveness of users in their job search by producing customized cover letters that meet linguistic and professional standards. Our tool will provide native-level language assistance and industry-appropriate expressions that are customized to each user's unique experience and needs. And with this solution, we hope to reduce the barriers faced by newcomers and marginalized groups, increase their employability, promote economic integration, and contribute to a more inclusive workforce. This AI-driven approach not only meets an unfilled need, but also puts us at the forefront of the market as we scale to support an increasingly globalized and diverse talent pool.

## Objective

The aim of this project is to develop a website integrated with Large Language Models (LLMs) in Rust to assist users in creating professional and customized cover letters to improve their job search prospects. It involves optimizing prompts to improve tool performance, as well as creating an interactive, user-friendly web interface using the framework. The project has several key milestones, which are explained in detail below from both the front-end and back-end perspectives.

### 1. Front-End Development

Given the project's features, we decided to use a website as our platform, and for a website to be effective, it's important to think about how users will navigate and interact with it. We needed to create a detailed UI/UX design document that would make sure the interface is easy to understand, accessible, and user-friendly. This document will describe how users will engage with the website and include all potential features, serving as a clear guide for development. On top of this, the front-end also needs to to include all the features and connect with the back-end API to allow for real-time updates and data syncing, ensuring a smooth content generation experience for the user. Our main goals can be summarized as follows:

1. **Improve user experience**: By designing an easy-to-use interface and providing clear guidance, users can quickly create a customized cover letter with just a few clicks and simple steps, making it easier for them to learn how to use the tool.
2. **Real-time feedback**: The front-end shows instant suggestions and reminders as users fill in their personal information, such as highlighting important skills, pointing out missing information, and providing interactive feedback to help adjust the length and tone of the writing. (Will be discussed later in feature section)
3. **Flexible component design**: The front-end framework is built in a way that allows for reusable and expandable components, so new features can be added easily in the future while keeping the code organized and easy to maintain.

![Our Front-End Design](https://github.com/Meghanxuxx/Rust_1724/blob/report/Frontend/static/UI.jpg?raw=true)

### 2. Back-End Development

Since the back-end needs to provide reliable support for the front-end's real-time interactions and data processing, our design aims to create clear and effective API interfaces that can quickly respond to user requests, ensures that the front-end receives accurate feedback. Meanwhile, since generating high-quality customized cover letters relies heavily on the prompt design, we will fine-tune and optimize the prompts to maximize the output quality, so that we can generate customized cover letter content that better matches the job requirements and personal background of the users. Finally, in order for users to view their past communications, the backend needs to satisfy storage functionality, including user account information as well as chat logs. So the objectives of our backend can be summarized as:

1. **Provide API Services**: Use clear endpoints (e.g., /api/step1, /api/step2, /api/step3) to receive multi-step input from the user, and after the user has completed submitting all the necessary information, send the integrated data to the LLM API for customized cover letter content and return it to the front-end.
2. **Optimize Prompt**: Perform basic processing and splicing of user inputs in the back-end, pass them to the LLM API as prompt words, and adjust the parameters according to the configuration file so that the model generates output content that better matches the job description and user background.
3. **Data Storage and Access Control**: Store the user's basic information and chat history in a local JSON file. Users can later revisit the data submitted and content generated in the past.

### 3. Project Objective Milestones Conclusion

The overall development of the project can be divided into the following key milestones:

1. **UI/UX Design**: Create and iteratively improve the design based on user feedback.
2. **Rust Front-End Framework**: Develop the front-end using a Rust framework.
3. **Front-End and Back-End API Interface**: Build a real-time interface for communication between the front-end and back-end.
4. **LLM API Integration**: Successfully incorporate the LLM API with optimized prompts.
5. **Prompt Engineering and Optimization**: Design and refine prompts for high-quality outputs.
6. **Performance Optimization and Testing**: Test the system extensively and make necessary optimizations to ensure optimal performance.

---
